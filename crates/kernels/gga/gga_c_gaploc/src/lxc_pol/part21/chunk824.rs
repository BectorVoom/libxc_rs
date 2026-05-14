//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 824/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk824<F: Float>(t7585: F, t8561: F, t1890: F, t2925: F, t590: F, t1445: F, t8612: F, t1628: F, t3066: F, t1043: F, t4598: F, t1029: F, t4585: F, t1048: F, t808: F, t8720: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8797 = t7585 * t8561;
    let t8802 = t1890 * t2925;
    let t8803 = t8802 * t590;
    let t8806 = t1445 * t8612;
    let t8809 = t1628 * t3066;
    let t8816 = t4598 * t1043;
    let t8819 = t4585 * t1029;
    let t8822 = t4598 * t1048;
    let t8827 = t808 * t8720;
    (t8797, t8802, t8803, t8806, t8809, t8816, t8819, t8822, t8827)
}
