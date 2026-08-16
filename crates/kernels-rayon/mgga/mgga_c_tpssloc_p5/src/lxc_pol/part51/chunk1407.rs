//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1407/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1407(t33428: f64, t6562: f64, t794: f64, t114790: f64, t7488: f64, t114965: f64, t118935: f64, t118938: f64, t118941: f64, t118944: f64, t1912: f64, t2053: f64, t25184: f64, t25329: f64, t25348: f64, t2718: f64, t31416: f64, t33398: f64, t7087: f64, t7107: f64, t855: f64, t865: f64, t92847: f64, t92939: f64, t98279: f64) -> f64 {
    let t121749 = t6562 * t794 * t33428;
    let t121753 = t6562 * t114790 * t7488;
    let t121770 = -0.41123351671205660912e-2_f64 * t121749 - t25348 * t7107 + 0.41123351671205660912e-2_f64 * t121753 + 2.0_f64 * t855 * t2718 * t33398 * t865 + 2.0_f64 * t855 * t2718 * t2053 * t25329 - t118935 - t92939 * t1912 - t118938 + t118941 + 0.41123351671205660912e-2_f64 * t114965 + 2.0_f64 * t7087 * t25184 - t92847 * t1912 - 6.0_f64 * t98279 * t31416 - t118944;
    t121770
}
