//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1508/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508<F: Float>(t3160: F, t42664: F, t11874: F, t16048: F, t1042: F, t11252: F, t11634: F, t11862: F, t11877: F, t2251: F, t3075: F, t3127: F, t3157: F, t3164: F, t42643: F, t42648: F, t42656: F, t42658: F, t42660: F, t42662: F, t42665: F, t42669: F, t4801: F) -> F {
    let t42672 = t42664 * t3160;
    let t42675 = t11874 * t16048;
    let t42678 = -F::cast_from(0.51448821741683684368e-2_f64) * t42643 * t11862 + F::cast_from(0.27439371595564631662e-1_f64) * t42648 * t11252 + F::cast_from(0.17149607247227894789e-2_f64) * t3127 * t1042 * t4801 * t2251 * t3075 + F::cast_from(0.30488190661738479624e-2_f64) * t42656 - F::cast_from(0.18292914397043087775e-1_f64) * t42658 - F::cast_from(0.18292914397043087775e-1_f64) * t42660 + F::cast_from(0.91464571985215438872e-2_f64) * t42662 + F::cast_from(0.25724410870841842184e-2_f64) * t42665 * t3157 + F::cast_from(0.51448821741683684368e-2_f64) * t42669 * t11634 - F::cast_from(0.12862205435420921092e-2_f64) * t42672 * t3164 - F::cast_from(0.13719685797782315831e-1_f64) * t42675 * t11877;
    t42678
}
