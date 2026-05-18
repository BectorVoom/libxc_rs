//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 932/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk932<F: Float>(t2033: F, t2365: F, t35451: F, t11784: F, t2679: F, t9800: F, t1445: F, t1457: F, t2004: F, t3025: F, t3451: F, t45091: F, t45772: F, t45775: F, t45778: F, t45785: F, t45792: F, t45793: F, t45794: F, t45795: F, t45798: F, t45801: F, t45803: F, t45806: F, t45809: F, t45812: F, t45817: F, t4752: F, t807: F) -> F {
    let t45819 = t2033 * t2365 * t35451;
    let t45820 = F::new(0.44688112439813033337e-1) * t45819;
    let t45822 = t9800 * t11784 * t2679;
    let t45823 = F::new(0.9585731488480187419e0) * t45822;
    let t45824 = -t45772 + t45775 - t45778 + F::new(0.23005755572352449806e1) * t807 * t1445 * t45091 + t45785 + F::new(0.35750489951850426669e0) * t2004 * t1457 * t45091 - t45792 - t45793 - t45794 - t45795 + t45798 + t45801 + t45803 + t45806 + t45809 + t45812 - F::new(0.14300195980740170668e1) * t3025 * t4752 * t3451 - t45817 - t45820 + t45823;
    t45824
}
