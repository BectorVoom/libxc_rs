//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 932/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk932<F: Float>(t1445: F, t2949: F, t813: F, t9688: F, t13130: F, t2194: F, t13157: F, t4673: F, t6060: F, t13129: F, t4614: F, t3271: F, t8556: F) -> (F, F, F, F, F) {
    let t43959 = F::new(0.46011511144704899612e1) * t813 * t1445 * t2949 * t9688;
    let t43961 = F::new(0.46011511144704899612e1) * t2194 * t13130;
    let t43972 = F::new(0.14300195980740170667e1) * t6060 * t4673 * t13157;
    let t43975 = F::new(0.61348681526273199483e1) * t813 * t4614 * t13129;
    let t43977 = F::new(0.23833659967900284446e0) * t3271 * t8556;
    (t43959, t43961, t43972, t43975, t43977)
}
