//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 802/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk802<F: Float>(t12916: F, t12961: F, t3675: F, t985: F, t3683: F, t3637: F, t967: F, t10168: F, t10170: F, t12906: F, t12907: F, t12913: F, t12931: F, t12934: F, t12946: F, t12947: F, t12951: F, t12960: F, t133: F, t2911: F, t2912: F, t5753: F, t5755: F, t5776: F, t5863: F, t8252: F) -> (F, F, F, F, F) {
    let t12962 = t12916 + t12961;
    let t12970 = t3675 * t985;
    let t12973 = t985 * t3683;
    let t12978 = t967 * t3637;
    let t12987 = -t5863 + t12934 - t12906 + t12907 - F::new(0.2069106e2) * t133 * t12913 + F::new(0.15518295e2) * t2911 * t2912 * t12978 + t5753 - t5755 - t12947 - t12951 - F::cast_from(0.51727649999999999999e1_f64) * t10168 + F::new(0.1724255e1) * t10170 - t5776 - t12960 + t12946 - F::new(0.1724255e1) * t133 * t12931 - F::cast_from(0.22990066666666666666e1_f64) * t8252;
    (t12962, t12970, t12973, t12978, t12987)
}
