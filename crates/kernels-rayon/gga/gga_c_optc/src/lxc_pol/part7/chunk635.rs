//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 635/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk635(t2849: f64, t3145: f64, t1897: f64, t894: f64, t1135: f64, t2855: f64, t1136: f64, t1900: f64, t1111: f64, t1121: f64, t1133: f64, t3081: f64, t3083: f64, t3089: f64, t3094: f64, t3098: f64, t3103: f64, t3110: f64, t3114: f64, t3116: f64, t3121: f64, t3129: f64, t3132: f64, t3134: f64, t3140: f64, t3142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3146 = t3145 * t2849;
    let t3147 = t3146 * t1897;
    let t3148 = t894 * t3147;
    let t3151 = t1135 * t2855;
    let t3152 = t3151 * t1897;
    let t3153 = t894 * t3152;
    let t3156 = t1136 * t1900;
    let t3157 = t894 * t3156;
    let t3160 = -t3081 + t3083 / 432.0_f64 + t1111 * t3089 / 216.0_f64 - t1111 * t3094 / 144.0_f64 + t1111 * t3098 / 288.0_f64 + 0.9157278480459830169e1_f64 * t3103 * t3110 + 0.47333755318775392234e-1_f64 * t3114 + 0.47333755318775392234e-1_f64 * t3116 * t3121 + 0.35500316489081544176e-1_f64 * t1121 * t3129 - 0.45786392402299150845e1_f64 * t3132 * t3134 - t3140 + 0.24147670804968771818e-2_f64 * t3142 + 0.30184588506210964773e-2_f64 * t1133 * t3148 - 0.36221506207453157728e-2_f64 * t1133 * t3153 + 0.18110753103726578864e-2_f64 * t1133 * t3157;
    (t3146, t3147, t3148, t3151, t3152, t3153, t3156, t3157, t3160)
}
