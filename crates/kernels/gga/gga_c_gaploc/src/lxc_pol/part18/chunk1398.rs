//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1398/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1398<F: Float>(t10513: F, t580: F, t587: F, t20592: F, t2487: F, t18676: F, t34459: F, t6711: F, t10314: F, t6710: F, t34246: F, t6717: F, t6914: F) -> (F, F, F, F, F) {
    let t34740 = F::cast_from(0.24539472610509279794e2_f64) * t587 * t580 * t10513;
    let t34743 = F::cast_from(0.11656249489991907902e3_f64) * t2487 * t20592 * t10513;
    let t34746 = F::cast_from(0.23005755572352449806e2_f64) * t18676 * t6711 * t34459;
    let t34749 = F::cast_from(0.30674340763136599742e2_f64) * t6710 * t20592 * t10314;
    let t34752 = F::cast_from(0.62115540045351614476e2_f64) * t6914 * t6717 * t34246;
    (t34740, t34743, t34746, t34749, t34752)
}
