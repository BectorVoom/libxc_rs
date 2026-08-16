//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1975/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1975<F: Float>(t1904: F, t28824: F, t689: F, t109407: F, t7289: F, t27884: F, t28845: F, t102255: F, t102257: F, t102261: F, t102266: F, t102270: F, t102272: F, t102276: F, t108653: F, t25924: F, t26304: F, t27837: F, t27868: F, t28792: F, t5774: F, t7295: F, t75016: F, t8094: F, t94823: F, t96277: F) -> F {
    let t109505 = t689 * t28824 * t1904;
    let t109512 = t7289 * t109407;
    let t109514 = t27884 * t28845;
    let t109516 = -t102255 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t28792 + F::cast_from(0.39029762157531132076e-1_f64) * t102257 + F::cast_from(0.26020884564615598386e1_f64) * t94823 * t26304 * t108653 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t8094 * t5774 + F::cast_from(0.10975748638225852664e-1_f64) * t109505 + t102261 - F::cast_from(0.96373646535613327357e-2_f64) * t96277 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t26304 * t75016 + F::cast_from(0.23131639038696784278e-2_f64) * t102266 - F::cast_from(0.12851425765524037203e-1_f64) * t109512 + t102270 - F::cast_from(0.25702851531048074406e-1_f64) * t109514 - t102272 - t102276;
    t109516
}
