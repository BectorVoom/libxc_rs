//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1550;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1551;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1552;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta378(t14142: f64, t4582: f64, t12648: f64, t4583: f64, t13559: f64, t977: f64, t2960: f64, t4603: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t1409: f64, t2244: f64, t10263: f64, t10403: f64, t1041: f64, t10413: f64, t10896: f64, t14122: f64, t14126: f64, t14130: f64, t14136: f64, t14139: f64, t1607: f64, t3070: f64, t3117: f64, t4562: f64, t4565: f64, t4585: f64, t10277: f64, t3061: f64, t12652: f64, t4588: f64, t10216: f64, t10969: f64, t135: f64, t4608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14143, t14147, t14152, t14158, t14159, t14160, t14164) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1550(t14142, t4582, t12648, t4583, t13559, t977, t2960, t4603, t1606, t698, t973, t1043, t2770);
        let t14165 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1551(t1409, t2244);
        let (t14167, t14170) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1552(t14164, t14165, t4582, t10263, t10403, t1041, t10413, t10896, t14122, t14126, t14130, t14136, t14139, t14143, t14147, t14152, t14158, t14160, t1607, t2960, t3070, t3117, t4562, t4565, t4585, t973);
        let (t14174, t14180, t14184, t14189, t14192, t14194) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1553(t10277, t3061, t14165, t4582, t12652, t4588, t12648, t10216, t10969, t135, t4608, t973);
    (t14143, t14147, t14159, t14165, t14167, t14170, t14174, t14180, t14184, t14189, t14192, t14194)
}
