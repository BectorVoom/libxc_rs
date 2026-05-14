//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1246/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1246<F: Float>(t1882: F, t26237: F, t26041: F, t487: F, t492: F, t102423: F, t102433: F, t11520: F, t11525: F, t11593: F, t11837: F, t1588: F, t1871: F, t1901: F, t1902: F, t23244: F, t26171: F, t26176: F, t26184: F, t26188: F, t26367: F, t26371: F, t26374: F, t3266: F, t446: F, t452: F, t47007: F, t47120: F, t480: F, t5630: F, t5644: F, t6564: F, t83: F, t8372: F, t92016: F, t925: F) -> (F, F) {
    let t103142 = 4.0 / 9.0 * t1882 * t26237;
    let t103163 = t26041 * t487;
    let t103164 = t103163 * t492;
    let t103190 = 2.0 / 3.0 * t446 * t452 * t11837 * t5644 - t103142 - 2.0 * t446 * t83 * t102433 + 4.0 / 3.0 * t446 * t83 * t102423 + 2.0 / 9.0 * t1901 * t8372 * t26184 + 4.0 / 9.0 * t11593 * t8372 * t26188 + t1901 * t1902 * t92016 * t925 / 9.0 + 2.0 / 3.0 * t446 * t1871 * t6564 * t1588 - 2.0 / 3.0 * t446 * t83 * t103164 - 4.0 / 3.0 * t1901 * t47120 * t26367 - 4.0 * t1901 * t26371 * t480 * t26374 - 4.0 * t1901 * t26171 * t23244 * t3266 - 4.0 * t1901 * t26171 * t5630 * t11520 - 2.0 * t1901 * t26171 * t5630 * t11525 - 4.0 / 3.0 * t1901 * t47007 * t26176;
    (t103164, t103190)
}
