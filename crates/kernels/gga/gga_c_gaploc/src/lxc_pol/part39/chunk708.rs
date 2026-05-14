//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 708/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk708<F: Float>(t13861: F, t1445: F, t833: F, t12218: F, t935: F, t2087: F, t12573: F, t12574: F, t13087: F, t13088: F, t13091: F, t13092: F, t13093: F, t13094: F, t13095: F) -> (F, F, F, F, F, F) {
    let t13862 = t1445 * t13861;
    let t13863 = t833 * t13862;
    let t13865 = t12218 * t935;
    let t13866 = t1445 * t13865;
    let t13867 = t2087 * t13866;
    let t13870 = t13087 + t13088 / 2.0 + t12573 - t12574 - t13091 - t13092 + t13093 + t13094 + t13095;
    (t13862, t13863, t13865, t13866, t13867, t13870)
}
