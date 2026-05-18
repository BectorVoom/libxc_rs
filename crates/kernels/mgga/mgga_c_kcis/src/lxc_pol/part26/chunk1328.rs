//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1328/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1328<F: Float>(t101950: F, t102729: F, t102731: F, t102733: F, t102735: F, t102740: F, t102743: F, t102746: F, t2256: F, t2260: F, t62923: F, t7986: F, t99667: F, t99671: F, t99676: F) -> F {
    let t102751 = -F::new(0.23214722222222222221e-2) * t102729 - F::new(0.25794135802469135802e-3) * t102731 - t99667 + F::new(0.23168402777777777778e-3) * t102733 + t99671 + F::new(0.23168402777777777778e-3) * t102735 + F::new(0.33980324074074074074e-2) * t101950 * t7986 + F::new(0.92858888888888888886e-2) * t102740 - F::new(0.92858888888888888886e-2) * t102743 + F::new(0.17024129629629629629e-1) * t102746 - F::new(0.34752604166666666667e-3) * t62923 * t2256 * t2260 + t99676;
    t102751
}
