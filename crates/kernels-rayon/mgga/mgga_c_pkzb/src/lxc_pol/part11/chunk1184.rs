//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1184/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1184(t29049: f64, t29065: f64, t10566: f64, t5221: f64, t10643: f64, t1769: f64, t10502: f64, t16379: f64, t164: f64, t16407: f64, t17034: f64, t179: f64, t19947: f64, t20065: f64, t24040: f64, t24054: f64, t24064: f64, t24075: f64, t24077: f64, t24087: f64, t24089: f64, t24096: f64, t2600: f64, t29024: f64, t29032: f64, t5279: f64, t600: f64, t612: f64, t615: f64, t616: f64) -> (f64, f64) {
    let t29067 = t29049 / 2.0_f64 + t29065 / 2.0_f64;
    let t29081 = t5221 * t10566;
    let t29084 = t1769 * t10643;
    let t29091 = 0.51448821741683684368e-2_f64 * t20065 * t179 * t29024 + 0.30011812682648815881e-2_f64 * t24040 + 0.34013387707001991332e0_f64 * t24054 + 0.45351183609335988442e0_f64 * t19947 + 455.0_f64 / 648.0_f64 * t16379 + 0.40015750243531754507e-2_f64 * t29032 - 0.85748036236139473944e-3_f64 * t612 * t615 * t616 * t29067 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t2600 * t24064 + 7.0_f64 / 4.0_f64 * t24075 - 7.0_f64 / 8.0_f64 * t24077 - 7.0_f64 / 16.0_f64 * t24087 + 35.0_f64 / 24.0_f64 * t24089 - 35.0_f64 / 72.0_f64 * t24096 - 7.0_f64 / 16.0_f64 * t29081 + 0.37792653007779990369e-1_f64 * t16407 - 0.60023625365297631763e-1_f64 * t29084 + 0.25724410870841842183e-1_f64 * t17034 * t179 * t10502 * t600 * t164;
    (t29067, t29091)
}
