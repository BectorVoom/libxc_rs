//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1140/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1140<F: Float>(t30239: F, t30382: F, t30697: F, t30730: F, t30734: F, t30739: F, t30742: F, t30745: F, t30747: F, t30749: F, t30751: F, t30753: F, t30755: F, t30787: F, t21933: F, t21935: F, t21951: F, t26513: F, t26527: F, t26535: F, t26585: F, t26588: F, t26592: F, t2945: F, t301: F, t757: F, t758: F, t761: F, t7796: F, t9194: F) -> (F, F) {
    let t30790 = t30239 + t30382 + t30730 - t30734 - t30739 - t30742 + t30745 - t30747 - t30749 - t30751 - t30753 - t30755 + t30697 + t30787;
    let t30803 = 0.42874018118069736972e-3 * t26513 - 0.1543464652250510531e-1 * t2945 * t758 * t7796 * t9194 + 0.21437009059034868486e-3 * t757 * t758 * t301 * t30790 * t761 + 0.19055119163586549765e-3 * t21933 - 0.45732285992607719437e-2 * t26527 + 0.76220476654346199061e-3 * t21935 - 0.14291339372689912324e-3 * t26535 + t26585 / 48.0 - t26588 / 16.0 + t26592 / 24.0 + t21951;
    (t30790, t30803)
}
