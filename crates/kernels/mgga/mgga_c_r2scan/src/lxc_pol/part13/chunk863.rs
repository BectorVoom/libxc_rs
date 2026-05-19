//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 863/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk863<F: Float>(t5818: F, t5821: F, t5930: F, t5932: F, t5934: F, t5936: F, t5940: F, t5945: F, t5950: F, t5952: F, t5955: F, t5959: F) -> F {
    let t7842 = -F::new(0.571528e-1) * t5930 + F::new(4.0) * t5932 + F::new(4.0) * t5934 - t5818 + t5821 + F::cast_from(0.1445810840040231979e-1_f64) * t5936 + t5940 + t5945 - t5950 - F::cast_from(0.20010214504933333333e-2_f64) * t5952 - F::cast_from(0.40020429009866666666e-2_f64) * t5955 + t5959;
    t7842
}
