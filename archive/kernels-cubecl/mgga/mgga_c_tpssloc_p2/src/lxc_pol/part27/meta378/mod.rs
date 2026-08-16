//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1550;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1551;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1552;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta378<F: Float>(t14142: F, t4582: F, t12648: F, t4583: F, t13559: F, t977: F, t2960: F, t4603: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t1409: F, t2244: F, t10263: F, t10403: F, t1041: F, t10413: F, t10896: F, t14122: F, t14126: F, t14130: F, t14136: F, t14139: F, t1607: F, t3070: F, t3117: F, t4562: F, t4565: F, t4585: F, t10277: F, t3061: F, t12652: F, t4588: F, t10216: F, t10969: F, t135: F, t4608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14143, t14147, t14152, t14158, t14159, t14160, t14164) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1550::<F>(t14142, t4582, t12648, t4583, t13559, t977, t2960, t4603, t1606, t698, t973, t1043, t2770);
        let t14165 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1551::<F>(t1409, t2244);
        let (t14167, t14170) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1552::<F>(t14164, t14165, t4582, t10263, t10403, t1041, t10413, t10896, t14122, t14126, t14130, t14136, t14139, t14143, t14147, t14152, t14158, t14160, t1607, t2960, t3070, t3117, t4562, t4565, t4585, t973);
        let (t14174, t14180, t14184, t14189, t14192, t14194) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1553::<F>(t10277, t3061, t14165, t4582, t12652, t4588, t12648, t10216, t10969, t135, t4608, t973);
    (t14143, t14147, t14159, t14165, t14167, t14170, t14174, t14180, t14184, t14189, t14192, t14194)
}
