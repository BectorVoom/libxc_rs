//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1420/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1420<F: Float>(t35336: F, t35339: F, t35349: F, t35352: F, t35355: F, t35358: F, t35361: F, t35341: F, t35343: F, t35346: F, t37285: F, t34315: F, t34330: F, t34335: F, t34338: F, t34341: F, t34480: F, t34739: F, t34971: F, t34980: F, t34987: F, t35105: F, t35110: F, t35113: F, t35115: F, t35119: F, t35266: F, t36896: F, t36897: F, t36898: F, t36899: F, t36900: F, t36917: F, t36930: F, t36942: F, t36945: F, t36946: F, t36947: F, t36948: F, t36949: F, t36950: F, t36951: F, t36952: F, t36953: F, t36967: F, t36980: F, t36992: F, t37006: F, t37018: F, t37031: F, t37043: F, t37046: F, t37047: F, t37048: F, t37049: F, t37050: F, t37051: F, t37052: F, t37053: F, t37054: F, t37068: F, t37081: F, t37093: F, t37108: F, t37120: F, t37133: F, t37145: F, t37149: F, t37150: F, t37151: F, t37153: F, t37154: F, t37156: F, t37157: F, t37170: F, t37183: F, t37195: F, t37200: F, t37205: F, t37206: F, t37207: F, t37208: F, t37221: F, t37234: F, t37246: F, t37249: F, t37250: F, t37251: F, t37252: F, t37253: F, t37254: F, t37255: F, t37256: F, t37257: F, t37271: F, t37284: F, t576: F) -> F {
    let t37286 = F::cast_from(0.2318836277704281739e-4_f64) * t35336;
    let t37287 = F::cast_from(0.11594181388521408695e-4_f64) * t35339;
    let t37291 = F::cast_from(0.19120257249055085362e-8_f64) * t35349;
    let t37292 = F::cast_from(0.12310223913928211462e-7_f64) * t35352;
    let t37293 = F::cast_from(0.16867947048611111112e-5_f64) * t35355;
    let t37294 = F::cast_from(0.14759453667534722223e-5_f64) * t35358;
    let t37295 = F::cast_from(0.20220636637604418766e-5_f64) * t35361;
    let t37296 = -t37285 + t37286 + t37287 + F::cast_from(0.3623181683912940217e-6_f64) * t35341 - F::cast_from(0.3623181683912940217e-6_f64) * t35343 - F::cast_from(0.28680385873582628044e-8_f64) * t35346 + t37291 - t37292 + t37293 - t37294 - t37295;
    let t37302 = t576 * (t37052 - t37053 - t37054 + t37031 + t37221 + t37081 - t36945 + t36946 + t36930 + t37195 + t37068 + t37018 - t36947 - t36948 - t36949 + t37043 + t37183 + t37046 + t37047 + t36992 + t37234 - t36899 + t36900 + t36980 + t37150 + t37151 + t37153 + t37284 + t37048 - t37049 + t37050 + t37051 + t37296 + t36896 - t36897 - t36898 - t37154 + t37156 - t37252 + t37253 - t37254 + t37108 - t37205 - t37206 + t37145 - t37207 + t37208 - F::cast_from(0.54347725258694103255e-6_f64) * t35115 - F::cast_from(0.90579542097823505425e-7_f64) * t34971 + t37255 - t37256 + t37257 - t37149 - t36950 + t36951 + t36952 - t36953 + t36967 + F::cast_from(0.67632724766374884053e-5_f64) * t35113 + t37120 + t37246 + F::cast_from(0.8839704917038230932e-8_f64) * t35119 + t37271 + F::cast_from(0.18115908419564701085e-6_f64) * t35110 + t36942 + F::cast_from(0.19666550313313802087e-7_f64) * t34739 - t37200 + t37249 + t37250 + t37251 - F::cast_from(0.8839704917038230932e-8_f64) * t34987 + F::cast_from(0.505954834707648426e-7_f64) * t34480 + t37133 + t37006 - F::cast_from(0.19908194100492367823e-6_f64) * t35105 - F::cast_from(0.19336854506021130163e-7_f64) * t35266 + t36917 - t37157 + t37170 + t37093 - F::cast_from(0.8839704917038230932e-8_f64) * t34980 + F::cast_from(0.18115908419564701085e-6_f64) * t34315 + F::cast_from(0.97817934710145362362e-6_f64) * t34330 + F::cast_from(0.28680385873582628044e-8_f64) * t34335 + F::cast_from(0.98332751566569010434e-7_f64) * t34338 + F::cast_from(0.57360771747165256087e-8_f64) * t34341);
    t37302
}
