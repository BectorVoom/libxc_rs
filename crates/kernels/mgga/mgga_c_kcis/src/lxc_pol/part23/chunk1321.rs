//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1321/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1321<F: Float>(t28326: F, t28878: F, t28881: F, t28884: F, t28887: F, t28889: F, t28891: F, t27154: F, t27327: F, t27330: F, t27719: F, t27725: F, t27728: F, t8: F, t91786: F, t93848: F, t93849: F, t93852: F, t99743: F, t99758: F, t99767: F, t99786: F) -> F {
    let t99790 = t28326 / F::new(8.0);
    let t99791 = t28878 / F::new(8.0);
    let t99792 = t28881 / F::new(8.0);
    let t99793 = t28884 / F::new(8.0);
    let t99794 = t28887 / F::new(8.0);
    let t99795 = t28889 / F::new(8.0);
    let t99796 = t28891 / F::new(8.0);
    let t99797 = t91786 + t8 * (t99743 + t99758 + t99767 + t99786) - t99790 - t99791 - t27330 - t27725 - t27728 + t27154 - t27327 + t93848 - t27719 - t99792 - t99793 - t93849 - t99794 + t99795 + t99796 + t93852;
    t99797
}
