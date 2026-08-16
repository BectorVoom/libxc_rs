//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1997;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1998;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta556<F: Float>(t12524: F, t7769: F, t20173: F, t1458: F, t6534: F, t3941: F, t1873: F, t4072: F, t3938: F, t7467: F, t671: F, t1401: F, t26135: F, t23877: F, t23880: F, t26509: F, t26523: F, t26533: F, t26535: F, t26537: F, t5376: F, t577: F, t7010: F, t3701: F, t6995: F, t1862: F, t31: F, t607: F, t7752: F, t1390: F, t22811: F, t2233: F, t2239: F, t601: F, t9238: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26539, t26541, t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1997::<F>(t12524, t7769, t20173, t1458, t6534, t3941, t1873, t4072, t3938, t7467, t671, t1401, t26135);
        let t26555 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1998::<F>(t1458, t23877, t23880, t26509, t26523, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7010);
        let (t31035, t31683, t33136, t34475, t39041, t39049, t39054) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1999::<F>(t3701, t6995, t1862, t31, t607, t7752, t1390, t22811, t2233, t2239, t601, t9238);
    (t26542, t26545, t26550, t26555, t31035, t31683, t33136, t34475, t39041, t39049, t39054)
}
