//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 888/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk888(t2026: f64, t9613: f64, t2099: f64, t3652: f64, t757: f64, t179: f64, t2068: f64, t3515: f64, t299: f64, t3542: f64, t5627: f64, t2096: f64, t2945: f64, t3635: f64, t3641: f64, t3653: f64, t3657: f64, t3662: f64, t3666: f64, t5713: f64, t5925: f64, t5941: f64, t735: f64, t771: f64, t7786: f64, t9591: f64, t9596: f64, t9600: f64, t9606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9614 = t2026 * t9613;
    let t9616 = t2099 * t3652;
    let t9617 = t757 * t9616;
    let t9622 = t179 * t2068 * t3515;
    let t9623 = t299 * t9622;
    let t9628 = t179 * t5627 * t3542;
    let t9629 = t299 * t9628;
    let t9631 = t735 * t3635 / 36.0_f64 + 0.12862205435420921092e-2_f64 * t2945 * t9591 - 0.51448821741683684368e-2_f64 * t2945 * t9596 + 0.25724410870841842184e-2_f64 * t2945 * t9600 + 0.11433071498151929859e-2_f64 * t5713 * t3657 - 0.14291339372689912324e-3_f64 * t9606 - 0.47637797908966374413e-4_f64 * t5941 - 0.11433071498151929859e-2_f64 * t2096 * t3653 - 0.22866142996303859718e-2_f64 * t5925 * t3641 + 0.28582678745379824648e-3_f64 * t9614 + 0.14291339372689912324e-3_f64 * t9617 + t7786 + 0.22866142996303859718e-2_f64 * t771 * t3666 - 0.28582678745379824648e-3_f64 * t9623 - 0.68598428988911579157e-2_f64 * t771 * t3662 + 0.85748036236139473947e-3_f64 * t9629;
    (t9614, t9616, t9617, t9622, t9623, t9628, t9629, t9631)
}
