//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 831/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk831<F: Float>(t245: F, t18587: F, t258: F, t18217: F, t18221: F, t18233: F, t18387: F, t18392: F, t18398: F, t18492: F, t18627: F, t18659: F, t18750: F, t18759: F, t1178: F, t18: F, t15625: F, t1577: F, t21: F, t267: F, t363: F, t4011: F, t4431: F, t5: F, t5186: F, t776: F, t920: F) -> (F,) {
    let t246 = 10000000.0 <= t245;
    let t18760 = t18587 * t258;
    let t18772 = 2.0 * t18760 - 2.0 * t18392 - 4.0 * t18233 + 8.0 * t18659 - 4.0 * t18221 + 4.0 * t18627 - 12.0 * t18217 + 8.0 * t18750 - 2.0 * t18398 + 4.0 * t18492 - 2.0 * t18387;
    let t18773 = t18759 + t18772;
    let t18783 = t1178 * t18;
    let t18793 = piecewise3(t246, 0.0, t5 * t18773 * t21 / 4.0 + t5 * t5186 * t363 / 4.0 + t5 * t4011 * t920 / 2.0 + t5 * t18783 * t1577 + t5 * t776 * t4431 / 4.0 + t5 * t267 * t15625 / 4.0);
    (t18793,)
}
