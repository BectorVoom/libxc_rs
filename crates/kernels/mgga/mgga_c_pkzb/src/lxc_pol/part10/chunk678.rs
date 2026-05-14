//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 678/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk678<F: Float>(t2946: F, t655: F, t758: F, t179: F, t2739: F, t780: F, t1130: F, t2067: F, t2071: F, t2085: F, t2101: F, t2104: F, t2919: F, t2922: F, t2925: F, t2933: F, t2940: F, t2945: F, t299: F, t757: F, t771: F) -> (F, F, F, F) {
    let t2947 = t2946 * t655;
    let t2948 = t758 * t2947;
    let t2952 = t179 * t780 * t2739;
    let t2955 = 0.21437009059034868486e-3 * t757 * t2919 - 0.21437009059034868486e-3 * t2922 * t2925 + 0.14291339372689912324e-3 * t2101 - 0.76220476654346199061e-3 * t2085 - t2067 - 0.28582678745379824648e-3 * t2071 - 0.42874018118069736972e-3 * t2104 * t2933 + 0.22866142996303859718e-2 * t771 * t1130 - 0.28582678745379824648e-3 * t2940 + 0.12862205435420921092e-2 * t2945 * t2948 - 0.42874018118069736972e-3 * t299 * t2952;
    (t2947, t2948, t2952, t2955)
}
