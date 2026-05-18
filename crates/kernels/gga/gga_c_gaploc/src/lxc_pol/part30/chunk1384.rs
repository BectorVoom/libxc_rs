//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1384/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1384<F: Float>(t34378: F, t6717: F, t6914: F, t10532: F, t10533: F, t2487: F, t6711: F, t2898: F, t6625: F, t10444: F, t1407: F, t34363: F, t587: F, t912: F) -> (F, F, F, F, F, F) {
    let t34573 = F::new(0.37959496694381542179e3) * t6914 * t6717 * t34378;
    let t34576 = F::new(0.38649669361552115674e3) * t10532 * t10533 * t34378;
    let t34579 = F::new(0.14953741122029092374e3) * t2487 * t6711 * t34378;
    let t34580 = t2898 * t6625;
    let t34581 = F::new(0.89376224879626066674e-1) * t34580;
    let t34582 = t1407 * t10444;
    let t34583 = F::new(0.38342925953920749676e0) * t34582;
    let t34585 = t587 * t912 * t34363;
    (t34573, t34576, t34579, t34581, t34583, t34585)
}
