//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2366/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2366<F: Float>(t22579: F, t7685: F, t1874: F, t55934: F, t12725: F, t6525: F, t26168: F, t6876: F, t25989: F, t83886: F, t25994: F, t4034: F) -> (F, F, F, F, F, F) {
    let t91763 = t7685 * t22579;
    let t91765 = F::cast_from(4.0_f64) * t55934 * t1874;
    let t91767 = F::cast_from(4.0_f64) * t12725 * t6525;
    let t91769 = F::cast_from(6.0_f64) * t6876 * t26168;
    let t91771 = F::cast_from(6.0_f64) * t83886 * t25989;
    let t91777 = F::cast_from(4.0_f64) * t4034 * t25994;
    (t91763, t91765, t91767, t91769, t91771, t91777)
}
