//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2446/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2446<F: Float>(t20234: F, t43070: F, t10236: F, t10186: F, t13851: F, t13861: F, t17804: F, t17817: F, t21413: F, t21430: F, t2986: F, t2988: F, t2990: F, t341: F, t43069: F, t4510: F, t4518: F, t4548: F, t5836: F, t68534: F, t68539: F, t68543: F, t68547: F, t69487: F, t69496: F, t69503: F, t69505: F, t69515: F) -> F {
    let t69519 = t43070 * t20234;
    let t69529 = t10236 * t20234;
    let t69533 = F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t13851 * t17817 - F::cast_from(0.44444444444444444443e-2_f64) * t10186 * t21430 + F::cast_from(0.55555555555555555553e-3_f64) * t69487 - F::cast_from(0.24999999999999999999e-2_f64) * t2986 * t341 * t5836 * t4548 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t17804 * t13861 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t69496 * t2990 - F::cast_from(0.29629629629629629629e-2_f64) * t10186 * t21413 + F::cast_from(0.37037037037037037037e-3_f64) * t69503 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t69505 * t2990 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4518 * t68534 + F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t4510 * t68539 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t2988 * t69515 - F::cast_from(0.86419753086419753084e-3_f64) * t2986 * t43069 * t69519 - F::cast_from(0.66666666666666666664e-2_f64) * t2986 * t4518 * t68543 + F::cast_from(0.49999999999999999998e-2_f64) * t2986 * t4518 * t68547 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t2988 * t69529;
    t69533
}
