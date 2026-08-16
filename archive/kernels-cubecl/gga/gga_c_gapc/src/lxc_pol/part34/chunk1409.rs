//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1409/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1409<F: Float>(t34911: F, t34914: F, t34918: F, t34926: F, t34929: F, t34934: F, t34936: F, t34905: F, t34907: F, t34921: F, t37124: F, t34940: F) -> (F, F) {
    let t37125 = F::cast_from(0.14759453667534722223e-5_f64) * t34911;
    let t37126 = F::cast_from(0.14759453667534722223e-5_f64) * t34914;
    let t37127 = F::cast_from(0.88465285289519332099e-6_f64) * t34918;
    let t37129 = F::cast_from(0.23333993417245370372e-3_f64) * t34926;
    let t37130 = F::cast_from(0.27012148473991046866e-5_f64) * t34929;
    let t37131 = F::cast_from(0.21915101773490614185e-6_f64) * t34934;
    let t37132 = F::cast_from(0.13506074236995523433e-5_f64) * t34936;
    let t37133 = -F::cast_from(0.49166375783284505217e-8_f64) * t34905 + F::cast_from(0.65555167711046006956e-8_f64) * t34907 - t37124 + t37125 + t37126 + t37127 + F::cast_from(0.44935166661611007236e-6_f64) * t34921 - t37129 + t37130 - t37131 + t37132;
    let t37134 = F::cast_from(0.16009199995585360443e-6_f64) * t34940;
    (t37133, t37134)
}
