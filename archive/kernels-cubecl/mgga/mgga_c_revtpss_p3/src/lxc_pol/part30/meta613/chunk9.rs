//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2116/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2116<F: Float>(t1843: F, t25832: F, t651: F, t10416: F, t7742: F, t13435: F, t2322: F, t28063: F, t1907: F, t3889: F, t25082: F, t8717: F) -> (F, F, F, F, F) {
    let t98426 = F::cast_from(2.0_f64) * t651 * t1843 * t25832;
    let t98428 = F::cast_from(2.0_f64) * t10416 * t7742;
    let t98430 = F::cast_from(4.0_f64) * t13435 * t7742;
    let t98432 = F::cast_from(4.0_f64) * t2322 * t28063;
    let t98436 = t1907 * t3889;
    let t98439 = F::cast_from(3.0_f64) * t25082 * t8717 * t98436;
    (t98426, t98428, t98430, t98432, t98439)
}
