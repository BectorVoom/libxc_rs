//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1206/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1206<F: Float>(t35814: F, t35816: F, t35818: F, t35827: F, t35829: F, t35837: F, t35844: F, t35848: F, t35850: F, t31603: F, t31605: F, t35823: F, t35833: F, t35835: F, t35841: F, t35846: F, t35853: F, t35856: F) -> F {
    let t37731 = F::new(0.16006300097412701803e-1) * t35814;
    let t37732 = F::new(0.42874018118069736972e-3) * t35816;
    let t37733 = F::new(0.28582678745379824648e-3) * t35818;
    let t37735 = F::new(0.28582678745379824648e-3) * t35827;
    let t37736 = F::new(0.16006300097412701803e-1) * t35829;
    let t37739 = F::new(0.25724410870841842184e-2) * t35837;
    let t37741 = F::new(0.42874018118069736972e-3) * t35844;
    let t37743 = F::new(0.16809375e0) * t35848;
    let t37744 = F::new(0.1120625e0) * t35850;
    let t37747 = F::new(13.0) / F::new(72.0) * t31603 + F::new(0.76220476654346199063e-2) * t31605 + t37731 + t37732 + t37733 + F::new(0.21437009059034868486e-3) * t35823 + t37735 - t37736 + F::new(0.37737710747524982483e-2) * t35833 - F::new(0.51448821741683684367e-2) * t35835 + t37739 + F::new(0.10718504529517434243e-2) * t35841 + t37741 - F::new(0.34299214494455789578e-1) * t35846 - t37743 - t37744 + F::new(0.4584375e-1) * t35853 + F::new(0.22921875e-1) * t35856;
    t37747
}
