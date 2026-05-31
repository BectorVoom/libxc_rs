//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1217/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1217<F: Float>(t12276: F, t50832: F, t1167: F, t2182: F, t13751: F, t13756: F, t14149: F, t14153: F, t14383: F, t14821: F, t2423: F, t2494: F, t3946: F, t4062: F, t4063: F, t50833: F, t50835: F, t52052: F, t52763: F, t52767: F, t52774: F, t52775: F, t52782: F) -> F {
    let t52789 = F::cast_from(6.0_f64) * t50832 * t12276;
    let t52791 = t1167 * t2182;
    let t52798 = -t1167 * t4062 * t52052 + F::cast_from(6.0_f64) * t13751 * t2494 * t3946 - F::cast_from(6.0_f64) * t13756 * t4063 * t52791 - F::cast_from(6.0_f64) * t14149 * t14383 * t3946 + F::cast_from(2.0_f64) * t14153 * t4062 * t52763 - t14821 * t2423 * t4062 - F::cast_from(6.0_f64) * t3946 * t4063 * t52767 - F::cast_from(6.0_f64) * t3946 * t4063 * t52782 + F::cast_from(12.0_f64) * t52774 * t52775 - F::cast_from(6.0_f64) * t50833 + F::cast_from(6.0_f64) * t50835 - t52789;
    t52798
}
