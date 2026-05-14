//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 840/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk840<F: Float>(t299: F, t9622: F, t179: F, t3542: F, t5627: F, t2096: F, t2945: F, t3635: F, t3641: F, t3653: F, t3657: F, t3662: F, t3666: F, t5713: F, t5925: F, t5941: F, t735: F, t771: F, t7786: F, t9591: F, t9596: F, t9600: F, t9606: F, t9614: F, t9617: F) -> (F, F, F, F) {
    let t9623 = t299 * t9622;
    let t9628 = t179 * t5627 * t3542;
    let t9629 = t299 * t9628;
    let t9631 = t735 * t3635 / 36.0 + 0.12862205435420921092e-2 * t2945 * t9591 - 0.51448821741683684368e-2 * t2945 * t9596 + 0.25724410870841842184e-2 * t2945 * t9600 + 0.11433071498151929859e-2 * t5713 * t3657 - 0.14291339372689912324e-3 * t9606 - 0.47637797908966374413e-4 * t5941 - 0.11433071498151929859e-2 * t2096 * t3653 - 0.22866142996303859718e-2 * t5925 * t3641 + 0.28582678745379824648e-3 * t9614 + 0.14291339372689912324e-3 * t9617 + t7786 + 0.22866142996303859718e-2 * t771 * t3666 - 0.28582678745379824648e-3 * t9623 - 0.68598428988911579157e-2 * t771 * t3662 + 0.85748036236139473947e-3 * t9629;
    (t9623, t9628, t9629, t9631)
}
