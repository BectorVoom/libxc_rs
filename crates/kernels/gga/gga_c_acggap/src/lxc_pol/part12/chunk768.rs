//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 768/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk768<F: Float>(t7805: F, t7849: F, t7853: F, t7862: F, t7809: F, t7813: F, t7817: F, t7820: F, t7823: F, t7825: F, t7829: F, t7833: F, t7837: F, t7840: F, t7845: F, t7847: F, t7856: F, t7864: F, t7868: F, t7872: F) -> (F, F, F, F, F) {
    let t8278 = F::new(0.41930789719472202758e-3) * t7805;
    let t8291 = F::new(77.0) / F::new(864.0) * t7849;
    let t8292 = F::new(35.0) / F::new(216.0) * t7853;
    let t8294 = t7862 / F::new(192.0);
    let t8298 = -t8278 + F::new(0.22921875e-1) * t7809 + F::new(0.1528125e-1) * t7813 + t7817 / F::new(32.0) + F::new(0.4584375e-1) * t7820 - F::new(0.34299214494455789578e-2) * t7823 + F::new(0.34299214494455789578e-2) * t7825 - t7829 / F::new(64.0) + F::new(0.31448092289604152069e-3) * t7833 + F::new(0.42874018118069736972e-3) * t7837 + F::new(0.62896184579208304138e-3) * t7840 + F::new(0.41930789719472202758e-3) * t7845 - F::new(0.42874018118069736972e-3) * t7847 + t8291 + t8292 + t7856 / F::new(48.0) + t8294 - F::new(7.0) / F::new(72.0) * t7864 + F::new(0.62896184579208304137e-2) * t7868 - F::new(0.94344276868812456206e-2) * t7872;
    (t8278, t8291, t8292, t8294, t8298)
}
