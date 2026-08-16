//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1997;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1998;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta556(t12524: f64, t7769: f64, t20173: f64, t1458: f64, t6534: f64, t3941: f64, t1873: f64, t4072: f64, t3938: f64, t7467: f64, t671: f64, t1401: f64, t26135: f64, t23877: f64, t23880: f64, t26509: f64, t26523: f64, t26533: f64, t26535: f64, t26537: f64, t5376: f64, t577: f64, t7010: f64, t3701: f64, t6995: f64, t1862: f64, t31: f64, t607: f64, t7752: f64, t1390: f64, t22811: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26539, t26541, t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1997(t12524, t7769, t20173, t1458, t6534, t3941, t1873, t4072, t3938, t7467, t671, t1401, t26135);
        let t26555 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1998(t1458, t23877, t23880, t26509, t26523, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7010);
        let (t31035, t31683, t33136, t34475, t39041, t39049, t39054) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1999(t3701, t6995, t1862, t31, t607, t7752, t1390, t22811, t2233, t2239, t601, t9238);
    (t26542, t26545, t26550, t26555, t31035, t31683, t33136, t34475, t39041, t39049, t39054)
}
