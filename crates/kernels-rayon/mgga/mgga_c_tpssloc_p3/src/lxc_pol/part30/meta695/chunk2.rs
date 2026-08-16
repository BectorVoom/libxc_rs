//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2221/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2221(t23185: f64, t28426: f64, t81914: f64, t25248: f64, t776: f64, t87642: f64, t98336: f64, t81575: f64, t87067: f64, t87078: f64, t92492: f64, t98325: f64, t98328: f64, t98330: f64, t98334: f64, t98339: f64, t98342: f64, t98345: f64, t98349: f64, t98353: f64, t98356: f64, t98359: f64) -> f64 {
    let t98363 = t23185 * t81914 * t28426;
    let t98367 = t87642 * t25248 * t98336 * t776;
    let t98370 = 0.3289868133696452873e-1_f64 * t98325 - 0.9869604401089358619e-1_f64 * t98328 - 0.11514538467937585055e0_f64 * t98330 + 0.82246703342411321825e-2_f64 * t98334 - 0.49348022005446793095e-1_f64 * t98339 + t87067 - t92492 - 0.41123351671205660912e-2_f64 * t98342 + 0.16449340668482264365e-1_f64 * t98345 - 0.16449340668482264365e-1_f64 * t98349 - 0.16449340668482264365e-1_f64 * t98353 + 0.82246703342411321825e-2_f64 * t98356 - 0.6579736267392905746e-1_f64 * t98359 + 0.16449340668482264365e-1_f64 * t81575 - 0.82246703342411321825e-2_f64 * t98363 - 0.19739208802178717238e0_f64 * t98367 - 0.23029076935875170111e0_f64 * t87078;
    t98370
}
