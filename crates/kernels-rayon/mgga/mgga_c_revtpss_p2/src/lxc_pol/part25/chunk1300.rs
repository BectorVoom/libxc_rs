//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1300/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1300(t1940: f64, t1963: f64, t2000: f64, t2403: f64, t25436: f64, t25440: f64, t25445: f64, t25752: f64, t25767: f64, t25778: f64, t25781: f64, t27158: f64, t4541: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64, t92742: f64, t92775: f64, t92822: f64, t93404: f64, t94276: f64, t94280: f64, t94286: f64, t94293: f64, t94297: f64, t94312: f64, t94316: f64, t94320: f64) -> f64 {
    let t94324 = 9.0_f64 * t4541 * t7087 * t25752 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t94276 + 9.0_f64 * t27158 * t94280 - 3.0_f64 / 2.0_f64 * t1940 * t92775 * t7207 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t94286 + 3.0_f64 * t1940 * t93404 * t25778 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t94293 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t94297 + 9.0_f64 / 2.0_f64 * t2403 * t25436 * t7200 + 9.0_f64 / 2.0_f64 * t2403 * t7087 * t25767 - 3.0_f64 * t1940 * t25440 * t25781 + 3.0_f64 * t92822 * t2000 - 3.0_f64 * t1940 * t92742 * t94312 + 3.0_f64 * t1940 * t25445 * t94316 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t94320;
    t94324
}
