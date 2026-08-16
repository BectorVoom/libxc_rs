//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta761(t3264: f64, t4782: f64, t6020: f64, t1671: f64, t18834: f64, t11185: f64, t21899: f64, t1670: f64, t3313: f64, t63588: f64, t18258: f64, t4781: f64, t14850: f64, t18677: f64, t14838: f64, t18680: f64, t15207: f64, t18640: f64, t4802: f64, t4824: f64, t64103: f64, t64292: f64, t71793: f64, t71795: f64, t71797: f64, t71800: f64, t71803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71806, t71809, t71811, t71814, t71817) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562(t3264, t4782, t6020, t1671, t18834, t11185, t21899, t1670, t3313, t63588, t18258, t4781);
        let (t71819, t71821, t71828) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563(t14850, t18677, t14838, t18680, t15207, t18640, t4802, t4824, t64103, t64292, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817);
    (t71806, t71809, t71811, t71814, t71817, t71819, t71821, t71828)
}
