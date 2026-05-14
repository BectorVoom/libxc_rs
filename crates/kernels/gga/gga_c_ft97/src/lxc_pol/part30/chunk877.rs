//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 877/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk877<F: Float>(t150060: F, t24432: F, t6118: F, t141314: F, t3886: F, t150034: F, t97181: F, t27775: F, t33460: F, t24437: F, t1091: F, t140714: F, t2354: F, t24543: F, t35350: F, t2: F, t35516: F) -> (F, F, F, F, F, F, F, F, F) {
    let t150062 = t6118 * t24432 * t150060;
    let t150064 = t141314 * t3886;
    let t150066 = t6118 * t24432 * t150064;
    let t150069 = t6118 * t97181 * t150034;
    let t150071 = t33460 * t27775;
    let t150073 = t24437 * t24432 * t150071;
    let t150077 = t6118 * t2354 * t140714 * t1091;
    let t150079 = t24543 * t35350;
    let t150081 = t2 * t35516;
    (t150062, t150064, t150066, t150069, t150071, t150073, t150077, t150079, t150081)
}
