//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1269/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1269(t111: f64, t34228: f64, t119824: f64, t119826: f64, t119830: f64, t119831: f64, t119835: f64, t119837: f64, t119839: f64, t119841: f64, t119844: f64, t119850: f64, t119852: f64, t119856: f64, t119858: f64, t123027: f64, t123028: f64, t27863: f64, t5361: f64, t672: f64, t7271: f64, t8916: f64) -> (f64, f64) {
    let t125100 = t34228 * t111;
    let t125103 = -2.0_f64 * t125100 * t672 - 4.0_f64 * t27863 * t7271 + t5361 * t8916 - t119824 - t119826 - t119830 + t119831 + t119835 - t119837 - t119839 - t119841 - t119844 - t119850 - t119852 - t119856 + t119858 - 2.0_f64 * t123027 + 6.0_f64 * t123028;
    (t125100, t125103)
}
