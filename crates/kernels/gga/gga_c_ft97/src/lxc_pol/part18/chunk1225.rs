//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1225/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1225<F: Float>(t6418: F, t94035: F, t102065: F, t102066: F, t102071: F, t102076: F, t102079: F, t102082: F, t102268: F, t11437: F, t1286: F, t1564: F, t1647: F, t22909: F, t22935: F, t25539: F, t25602: F, t25861: F, t28: F, t3103: F, t3289: F, t432: F, t497: F, t5495: F, t5501: F, t5507: F, t5618: F, t93968: F) -> (F,) {
    let t102270 = t94035 * t6418;
    let t102283 = t1286 * t28 * t5618 * t3289 / 3.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t3289 * t432 + 2.0 / 9.0 * t93968 - 5.0 / 81.0 * t5501 * t102065 * t102066 * t11437 + 2.0 / 9.0 * t5501 * t102071 * t22909 + 2.0 / 27.0 * t102076 - t102079 - t102082 + t5495 * t25539 / 3.0 - 2.0 * t102268 - 2.0 / 81.0 * t102270 + 2.0 / 9.0 * t22935 * t25602 + t5501 * t1564 * t25861 * t1647 / 9.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t497 * t3103;
    (t102283,)
}
