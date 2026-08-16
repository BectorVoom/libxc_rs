//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2254/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2254(t81912: f64, t87412: f64, t87426: f64, t92676: f64, t92677: f64, t92689: f64, t98818: f64, t98820: f64, t98822: f64, t98824: f64, t98826: f64, t98828: f64, t98830: f64, t98833: f64, t98836: f64, t98838: f64, t98842: f64, t98844: f64) -> f64 {
    let t98846 = t92676 - t92677 + t87412 - t98818 / 384.0_f64 - t98820 / 384.0_f64 - t98822 / 192.0_f64 - t98824 / 192.0_f64 + 5.0_f64 / 384.0_f64 * t98826 - 35.0_f64 / 576.0_f64 * t98828 + 7.0_f64 / 288.0_f64 * t98830 - t98833 / 384.0_f64 - 0.11304371706359309439e-1_f64 * t81912 - 0.28260929265898273598e-2_f64 * t98836 - t87426 - t92689 - 0.16956557559538964158e-1_f64 * t98838 - 0.12111826828242117256e-2_f64 * t98842 + t98844 / 192.0_f64;
    t98846
}
