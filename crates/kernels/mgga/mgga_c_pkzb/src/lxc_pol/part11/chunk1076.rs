//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1076/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1076<F: Float>(t10643: F, t1769: F, t10502: F, t16379: F, t164: F, t16407: F, t17034: F, t179: F, t19947: F, t20065: F, t24040: F, t24054: F, t24064: F, t24075: F, t24077: F, t24087: F, t24089: F, t24096: F, t2600: F, t29024: F, t29032: F, t29067: F, t29081: F, t5279: F, t600: F, t612: F, t615: F, t616: F) -> (F,) {
    let t29084 = t1769 * t10643;
    let t29091 = 0.51448821741683684368e-2 * t20065 * t179 * t29024 + 0.30011812682648815881e-2 * t24040 + 0.34013387707001991332e0 * t24054 + 0.45351183609335988442e0 * t19947 + 455.0 / 648.0 * t16379 + 0.40015750243531754507e-2 * t29032 - 0.85748036236139473944e-3 * t612 * t615 * t616 * t29067 - 0.12862205435420921092e-1 * t5279 * t179 * t2600 * t24064 + 7.0 / 4.0 * t24075 - 7.0 / 8.0 * t24077 - 7.0 / 16.0 * t24087 + 35.0 / 24.0 * t24089 - 35.0 / 72.0 * t24096 - 7.0 / 16.0 * t29081 + 0.37792653007779990369e-1 * t16407 - 0.60023625365297631763e-1 * t29084 + 0.25724410870841842183e-1 * t17034 * t179 * t10502 * t600 * t164;
    (t29091,)
}
