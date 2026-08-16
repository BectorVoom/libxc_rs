//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1412/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1412(t3720: f64, t701: f64, t12214: f64, t13846: f64, t1890: f64, t1986: f64, t2103: f64, t22767: f64, t28022: f64, t32748: f64, t32753: f64, t32756: f64, t32759: f64, t32761: f64, t32764: f64, t32766: f64, t32769: f64, t32771: f64, t32774: f64, t38907: f64, t4673: f64, t5241: f64, t5640: f64, t5840: f64, t590: f64, t739: f64) -> (f64, f64) {
    let t38912 = t3720 * t701;
    let t38924 = -t32748 + 0.30674340763136599742e1_f64 * t5640 * t5241 * t38907 * t590 + 0.1022478025437886658e1_f64 * t5840 * t1890 * t38912 * t590 - 0.1022478025437886658e1_f64 * t1986 * t739 * t13846 * t22767 - t28022 + 0.95334639871601137784e0_f64 * t2103 * t4673 * t12214 + t32753 - t32756 - t32759 + t32761 - t32764 - t32766 + t32769 + t32771 + t32774;
    (t38912, t38924)
}
