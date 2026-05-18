//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 866/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk866<F: Float>(t12970: F, t13009: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F, t12999: F, t13000: F, t13002: F, t13005: F) -> (F, F) {
    let t13010 = t13009 * t12970;
    let t13014 = -F::new(0.17938e1) * t12959 + F::new(0.16431333333333333333e0) * t12962 - F::new(0.49293999999999999999e0) * t12965 - F::new(0.32862666666666666666e0) * t12967 - F::new(0.76790625e-1) * t12971 + F::new(0.1898925e1) * t12993 + F::new(0.3071625e0) * t12995 - t12999 - t13000 - F::new(0.82156666666666666668e-1) * t13002 + F::new(0.49293999999999999999e0) * t13005 + F::new(0.142419375e1) * t13010 - F::new(0.59793333333333333333e0) * t12985 + F::new(0.17938e1) * t12989;
    (t13010, t13014)
}
