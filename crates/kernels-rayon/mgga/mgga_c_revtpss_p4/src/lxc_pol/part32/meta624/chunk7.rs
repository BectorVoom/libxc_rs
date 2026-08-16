//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1975/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1975(t1904: f64, t28824: f64, t689: f64, t109407: f64, t7289: f64, t27884: f64, t28845: f64, t102255: f64, t102257: f64, t102261: f64, t102266: f64, t102270: f64, t102272: f64, t102276: f64, t108653: f64, t25924: f64, t26304: f64, t27837: f64, t27868: f64, t28792: f64, t5774: f64, t7295: f64, t75016: f64, t8094: f64, t94823: f64, t96277: f64) -> f64 {
    let t109505 = t689 * t28824 * t1904;
    let t109512 = t7289 * t109407;
    let t109514 = t27884 * t28845;
    let t109516 = -t102255 + 0.8673628188205199462e0_f64 * t27837 * t28792 + 0.39029762157531132076e-1_f64 * t102257 + 0.26020884564615598386e1_f64 * t94823 * t26304 * t108653 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t8094 * t5774 + 0.10975748638225852664e-1_f64 * t109505 + t102261 - 0.96373646535613327357e-2_f64 * t96277 + 0.4336814094102599731e0_f64 * t27868 * t26304 * t75016 + 0.23131639038696784278e-2_f64 * t102266 - 0.12851425765524037203e-1_f64 * t109512 + t102270 - 0.25702851531048074406e-1_f64 * t109514 - t102272 - t102276;
    t109516
}
