//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1649;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta437<F: Float>(t12238: F, t3428: F, t3376: F, t3432: F, t3436: F, t12358: F, t3379: F, t12571: F, t3539: F, t45021: F, t45023: F, t45026: F, t45029: F, t45033: F, t45037: F, t45040: F, t45043: F, t3453: F, t3488: F, t3495: F, t1175: F, t12485: F, t3444: F, t3476: F, t1156: F, t12469: F, t3450: F, t3475: F, t426: F, t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F, t43783: F, t43785: F, t43787: F, t43791: F, t43795: F, t43799: F, t43802: F, t43804: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45045, t45048, t45050, t45052, t45053) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1648::<F>(t12238, t3428, t3376, t3432, t3436, t12358, t3379, t12571, t3539, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043);
        let (t45057, t45061, t45064, t45075, t45080, t45085) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1649::<F>(t3453, t3488, t3495, t1175, t12485, t3444, t3476, t1156, t12469, t3450, t3475, t426);
        let t45103 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650::<F>(t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795, t43799, t43802, t43804);
    (t45045, t45048, t45050, t45052, t45053, t45057, t45061, t45064, t45075, t45080, t45085, t45103)
}
