//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 652/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk652(t2946: f64, t655: f64, t758: f64, t179: f64, t2739: f64, t780: f64, t1130: f64, t2067: f64, t2071: f64, t2085: f64, t2101: f64, t2104: f64, t2919: f64, t2922: f64, t2925: f64, t2933: f64, t2940: f64, t2945: f64, t299: f64, t757: f64, t771: f64) -> (f64, f64, f64, f64) {
    let t2947 = t2946 * t655;
    let t2948 = t758 * t2947;
    let t2952 = t179 * t780 * t2739;
    let t2955 = 0.21437009059034868486e-3_f64 * t757 * t2919 - 0.21437009059034868486e-3_f64 * t2922 * t2925 + 0.14291339372689912324e-3_f64 * t2101 - 0.76220476654346199061e-3_f64 * t2085 - t2067 - 0.28582678745379824648e-3_f64 * t2071 - 0.42874018118069736972e-3_f64 * t2104 * t2933 + 0.22866142996303859718e-2_f64 * t771 * t1130 - 0.28582678745379824648e-3_f64 * t2940 + 0.12862205435420921092e-2_f64 * t2945 * t2948 - 0.42874018118069736972e-3_f64 * t299 * t2952;
    (t2947, t2948, t2952, t2955)
}
