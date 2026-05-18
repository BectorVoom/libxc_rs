//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 898/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk898<F: Float>(t10845: F, t2487: F, t10794: F, t10799: F, t10803: F, t10807: F, t10812: F, t10816: F, t10820: F, t10824: F, t10826: F, t10828: F, t10833: F, t10838: F, t10842: F, t2745: F, t4362: F, t825: F, t851: F) -> F {
    let t10846 = t10845 * t2487;
    let t10848 = F::new(0.25724410870841842183e-2) * t2745 * t10794 + F::new(0.12862205435420921092e-2) * t4362 * t10799 + F::new(0.25724410870841842183e-2) * t2745 * t10803 - F::new(0.64311027177104605458e-3) * t2745 * t10807 - F::new(0.24009450146119052704e-1) * t10812 - F::new(0.17006693853500995666e-1) * t10816 + F::new(0.12862205435420921092e-1) * t851 * t10820 - t10824 + t10826 - F::new(0.21437009059034868486e-3) * t825 * t10828 - F::new(0.38115002106963996168e-4) * t10833 - F::new(0.17149607247227894789e-3) * t10838 - F::new(0.38115002106963996168e-4) * t10842 + F::new(0.40656002247428262579e-3) * t10846;
    t10848
}
