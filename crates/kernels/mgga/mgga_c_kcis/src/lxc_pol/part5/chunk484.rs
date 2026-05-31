//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 484/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk484<F: Float>(t1396: F, t2001: F, t1468: F, t1464: F, t1929: F, t556: F, t553: F, t303: F, t1650: F, t8: F, t168: F) -> (F, F, F, F, F, F, F) {
    let t2002 = t1396 * t2001;
    let t2003 = t1468 * t2002;
    let t2004 = t1464 * t2003;
    let t2006 = t1929 * t556;
    let t2007 = t553 * t2006;
    let t2008 = t303 * t2007;
    let t2010 = t8 * t1650;
    let t2011 = F::cast_from(1.0_f64) - t168 + t2010;
    (t2002, t2003, t2004, t2006, t2007, t2008, t2011)
}
