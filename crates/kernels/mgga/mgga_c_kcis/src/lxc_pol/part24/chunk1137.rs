//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1137/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1137<F: Float>(t2811: F, t6539: F, t1008: F, t26686: F, t13376: F, t1662: F, t4947: F, t14554: F, t4621: F, t4781: F, t27819: F, t6276: F, t100142: F, t100145: F, t100148: F, t100174: F, t18508: F, t26679: F, t26685: F, t26748: F, t27832: F, t27958: F, t28939: F, t7703: F) -> (F, F, F, F, F) {
    let t101001 = t2811 * t6539;
    let t101003 = t26686 * t101001 * t1008;
    let t101012 = t4947 * t13376 * t1662;
    let t101018 = t14554 * t4781 * t4621;
    let t101028 = t4947 * t27819 * t6276 * t1008;
    let t101031 = -0.69505208333333333333e-3 * t7703 * t101003 + 0.46336805555555555556e-3 * t26748 * t28939 + 0.46336805555555555557e-3 * t27832 * t27958 - 0.27636574074074074073e-2 * t100142 + 0.61836467013888888889e-4 * t26685 * t101012 + 0.18424382716049382715e-2 * t100145 - 0.16581944444444444444e-1 * t100148 + 0.12367293402777777778e-3 * t26685 * t101018 + 0.46336805555555555556e-3 * t7703 * t4947 * t26679 * t18508 - 0.33163888888888888888e-2 * t100174 + 0.30918233506944444445e-4 * t26685 * t101028;
    (t101003, t101012, t101018, t101028, t101031)
}
