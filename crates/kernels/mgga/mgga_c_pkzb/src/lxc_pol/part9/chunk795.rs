//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 795/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk795<F: Float>(t299: F, t5674: F, t2054: F, t2057: F, t2082: F, t2091: F, t276: F, t5630: F, t5637: F, t5641: F, t5646: F, t5649: F, t5658: F, t5661: F, t5666: F, t735: F, t744: F, t782: F) -> (F, F) {
    let t5675 = t299 * t5674;
    let t5677 = F::new(0.25724410870841842184e-2) * t5630 - F::new(0.51448821741683684368e-2) * t299 * t5637 - F::new(0.42874018118069736972e-3) * t299 * t5641 - t5646 / F::new(96.0) - t276 * t5649 / F::new(96.0) - F::new(11.0) / F::new(36.0) * t2057 * t744 - t735 * t2091 / F::new(6.0) - t276 * t5658 / F::new(16.0) + t5661 / F::new(18.0) + t5666 / F::new(48.0) + t735 * t2054 / F::new(12.0) - F::new(0.43445671692977333464e-1) * t2082 * t782 + F::new(0.28582678745379824648e-3) * t5675;
    (t5675, t5677)
}
