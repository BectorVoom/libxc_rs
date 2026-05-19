//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1044/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1044<F: Float>(t33682: F, t8337: F, t2404: F, t7924: F, t33839: F, t33841: F, t33843: F, t33851: F, t33859: F, t33886: F, t33903: F, t33940: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36809 = t33682 * t8337;
    let t36811 = t7924 * t2404;
    let t36817 = F::cast_from(0.31448092289604152068e-2_f64) * t33839;
    let t36818 = F::cast_from(0.37737710747524982482e-2_f64) * t33841;
    let t36819 = F::cast_from(0.62896184579208304138e-3_f64) * t33843;
    let t36821 = F::cast_from(0.41930789719472202758e-3_f64) * t33851;
    let t36824 = F::new(11.0) / F::new(288.0) * t33859;
    let t36836 = F::cast_from(0.57165357490759649296e-3_f64) * t33886;
    let t36841 = F::cast_from(0.57165357490759649296e-3_f64) * t33903;
    let t36870 = F::cast_from(0.21437009059034868486e-2_f64) * t33940;
    (t36809, t36811, t36817, t36818, t36819, t36821, t36824, t36836, t36841, t36870)
}
