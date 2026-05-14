//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1194/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1194<F: Float>(t34929: F, t34934: F, t34936: F, t34905: F, t34907: F, t34921: F, t37124: F, t37125: F, t37126: F, t37127: F, t37129: F, t34940: F, t34942: F, t34946: F, t34951: F, t34956: F) -> (F, F, F, F, F, F) {
    let t37130 = 0.27012148473991046866e-5 * t34929;
    let t37131 = 0.21915101773490614185e-6 * t34934;
    let t37132 = 0.13506074236995523433e-5 * t34936;
    let t37133 = -0.49166375783284505217e-8 * t34905 + 0.65555167711046006956e-8 * t34907 - t37124 + t37125 + t37126 + t37127 + 0.44935166661611007236e-6 * t34921 - t37129 + t37130 - t37131 + t37132;
    let t37134 = 0.16009199995585360443e-6 * t34940;
    let t37135 = 0.40518222710986570299e-5 * t34942;
    let t37136 = 0.17679409834076461864e-7 * t34946;
    let t37138 = 0.50603841145833333336e-5 * t34951;
    let t37140 = 0.26519114751114692796e-6 * t34956;
    (t37133, t37134, t37135, t37136, t37138, t37140)
}
