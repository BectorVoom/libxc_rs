//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 635/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk635<F: Float>(t2849: F, t3145: F, t1897: F, t894: F, t1135: F, t2855: F, t1136: F, t1900: F, t1111: F, t1121: F, t1133: F, t3081: F, t3083: F, t3089: F, t3094: F, t3098: F, t3103: F, t3110: F, t3114: F, t3116: F, t3121: F, t3129: F, t3132: F, t3134: F, t3140: F, t3142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3146 = t3145 * t2849;
    let t3147 = t3146 * t1897;
    let t3148 = t894 * t3147;
    let t3151 = t1135 * t2855;
    let t3152 = t3151 * t1897;
    let t3153 = t894 * t3152;
    let t3156 = t1136 * t1900;
    let t3157 = t894 * t3156;
    let t3160 = -t3081 + t3083 / F::new(432.0) + t1111 * t3089 / F::new(216.0) - t1111 * t3094 / F::new(144.0) + t1111 * t3098 / F::new(288.0) + F::new(0.9157278480459830169e1) * t3103 * t3110 + F::new(0.47333755318775392234e-1) * t3114 + F::new(0.47333755318775392234e-1) * t3116 * t3121 + F::new(0.35500316489081544176e-1) * t1121 * t3129 - F::new(0.45786392402299150845e1) * t3132 * t3134 - t3140 + F::new(0.24147670804968771818e-2) * t3142 + F::new(0.30184588506210964773e-2) * t1133 * t3148 - F::new(0.36221506207453157728e-2) * t1133 * t3153 + F::new(0.18110753103726578864e-2) * t1133 * t3157;
    (t3146, t3147, t3148, t3151, t3152, t3153, t3156, t3157, t3160)
}
