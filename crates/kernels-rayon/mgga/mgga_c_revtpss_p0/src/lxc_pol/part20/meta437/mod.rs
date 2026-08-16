//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1649;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta437(t12238: f64, t3428: f64, t3376: f64, t3432: f64, t3436: f64, t12358: f64, t3379: f64, t12571: f64, t3539: f64, t45021: f64, t45023: f64, t45026: f64, t45029: f64, t45033: f64, t45037: f64, t45040: f64, t45043: f64, t3453: f64, t3488: f64, t3495: f64, t1175: f64, t12485: f64, t3444: f64, t3476: f64, t1156: f64, t12469: f64, t3450: f64, t3475: f64, t426: f64, t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43791: f64, t43795: f64, t43799: f64, t43802: f64, t43804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45045, t45048, t45050, t45052, t45053) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1648(t12238, t3428, t3376, t3432, t3436, t12358, t3379, t12571, t3539, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043);
        let (t45057, t45061, t45064, t45075, t45080, t45085) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1649(t3453, t3488, t3495, t1175, t12485, t3444, t3476, t1156, t12469, t3450, t3475, t426);
        let t45103 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650(t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795, t43799, t43802, t43804);
    (t45045, t45048, t45050, t45052, t45053, t45057, t45061, t45064, t45075, t45080, t45085, t45103)
}
