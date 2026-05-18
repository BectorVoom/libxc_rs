//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1405/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1405<F: Float>(t34755: F, t34757: F, t34759: F, t34761: F, t34764: F, t34767: F, t34742: F, t37058: F, t37059: F, t37060: F, t37061: F, t34779: F) -> (F, F) {
    let t37062 = F::new(0.40483072916666666668e-3) * t34755;
    let t37063 = F::new(0.4637672555408563478e-4) * t34757;
    let t37064 = F::new(0.65654527540950461132e-8) * t34759;
    let t37065 = F::new(0.69685742139248181696e-4) * t34761;
    let t37066 = F::new(0.63350674672043801542e-5) * t34764;
    let t37067 = F::new(0.63350674672043801542e-5) * t34767;
    let t37068 = -F::new(0.98332751566569010434e-7) * t34742 - t37058 - t37059 - t37060 + t37061 + t37062 - t37063 + t37064 - t37065 - t37066 - t37067;
    let t37072 = F::new(0.13913017666225690434e-3) * t34779;
    (t37068, t37072)
}
