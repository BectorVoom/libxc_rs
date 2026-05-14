//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1265/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1265<F: Float>(t22892: F, t6414: F, t100127: F, t102431: F, t102433: F, t102436: F, t102439: F, t102442: F, t102500: F, t102550: F, t102604: F, t102654: F, t102700: F, t102748: F, t102792: F, t102839: F, t102891: F, t102938: F, t102992: F, t103035: F, t103085: F, t103135: F, t103190: F, t103231: F, t103283: F, t103339: F, t103363: F, t103416: F, t103471: F, t103508: F, t103548: F, t103597: F, t103641: F, t103687: F, t103723: F, t103775: F, t103816: F, t103859: F, t103906: F, t103947: F, t10974: F, t1564: F, t1761: F, t25553: F, t25579: F, t3051: F, t5494: F, t5495: F, t5501: F, t6562: F, t88: F, t925: F, t93931: F, t94046: F, t94049: F) -> (F,) {
    let t103955 = t6414 * t22892 / 9.0;
    let t103957 = 8.0 * t102431 - 12.0 * t102433 - 12.0 * t102436 + t102439 + t5495 * t25553 / 3.0 + 4.0 / 27.0 * t100127 * t102442 * t10974 + 4.0 / 27.0 * t94046 + 4.0 / 27.0 * t94049 - 2.0 / 9.0 * t5494 * t3051 * t25579 - t5501 * t1564 * t93931 * t925 / 18.0 - t88 * (t102700 + t103947 + t103363 + t103687 + t102550 + t102748 + t103641 + t103035 + t102792 + t102992 + t103775 + t102938 + t103190 + t103471 + t103339 + t103906 + t103231 + t102500 + t103416 + t103283 + t102839 + t103816 + t103508 + t103085 + t102654 + t102604 + t103859 + t103135 + t103548 + t103597 + t103723 + t102891) - t103955 - t1761 * t6562;
    (t103957,)
}
