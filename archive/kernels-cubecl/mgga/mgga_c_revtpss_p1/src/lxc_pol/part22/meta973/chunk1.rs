//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3261/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261<F: Float>(t18352: F, t2710: F, t2713: F, t10722: F, t6030: F, t18419: F, t9775: F, t14791: F, t14802: F, t40679: F, t40681: F, t40691: F, t40707: F, t40711: F, t40722: F, t4362: F, t50703: F, t50706: F, t6022: F) -> F {
    let t61888 = t2710 * t2713 * t18352;
    let t61890 = t10722 * t6030;
    let t61892 = t9775 * t18419;
    let t61899 = F::cast_from(0.65057734796334705782e-3_f64) * t50703 - F::cast_from(0.2032800112371413129e-3_f64) * t50706 + F::cast_from(0.15244095330869239812e-3_f64) * t40679 - F::cast_from(0.27104001498285508386e-2_f64) * t40681 + F::cast_from(0.22589491248727328396e-6_f64) * t40691 - F::cast_from(0.22675591804667994221e-1_f64) * t40707 - F::cast_from(0.10276933901433255263e-1_f64) * t40711 + F::cast_from(0.90357964994909313586e-4_f64) * t61888 - F::cast_from(0.22675591804667994221e-1_f64) * t61890 - F::cast_from(0.76220476654346199061e-4_f64) * t61892 - F::cast_from(0.3659040202268543632e-3_f64) * t40722 - F::cast_from(0.10289764348336736874e-1_f64) * t4362 * t14791 * t6022 * t14802;
    t61899
}
