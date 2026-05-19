//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 953/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk953<F: Float>(t772: F, t29754: F, t2021: F, t1586: F, t2642: F, t9176: F, t12255: F, t28368: F, t5486: F, t1775: F, t2629: F, t9183: F, t2633: F, t9207: F) -> (F, F, F, F, F) {
    let t783 = F::new(0.0) < t772;
    let t29845 = piecewise3::<F>(t783, t29754, -t29754);
    let t29846 = t2021 * t29845;
    let t29847 = t1586 * t29846;
    let t29852 = t9176 * t2642;
    let t29853 = t12255 * t29852;
    let t29854 = t1586 * t29853;
    let t29861 = t5486 * t28368;
    let t29862 = t1775 * t29861;
    let t29867 = t2629 * t9183;
    let t29870 = t9207 * t2633;
    (t29847, t29854, t29862, t29867, t29870)
}
