//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1089/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1089<F: Float>(t124: F, t1384: F, t1450: F, t1451: F, t1453: F, t1454: F, t1492: F, t1498: F, t1502: F, t18904: F, t19548: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t377: F, t446: F, t452: F, t454: F, t4726: F, t4729: F, t4776: F, t4780: F, t4812: F, t4855: F, t4859: F, t4860: F, t4863: F, t518: F, t625: F) -> (F,) {
    let t19650 = -0.46785788981077169656e1 * t1498 * t4855 * t452 + 0.6233709278045326953e3 * t4860 * t18904 * t1384 + 0.96491876992155210402e2 * t1451 * t19548 * t1453 + 0.44060335298551228073e1 * t625 * t124 * t1450 * t1454 + 0.13218100589565368422e2 * t625 * t377 * t4726 * t4729 - t19611 - t19614 - 0.1301229756036208781e0 * t625 * t4776 * t4812 + t19620 - t19624 + t19628 - 0.38025319932552508021e2 * t625 * t377 * t4859 * t4863 + 0.43374325201206959368e-1 * t625 * t4780 * t1502 - 0.21687162600603479684e-1 * t625 * t1492 * t4855 - 0.67471172535210825684e-1 * t625 * t518 * t446 * t454 + t19646 + t19649;
    (t19650,)
}
